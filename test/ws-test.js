import { check, fail } from 'k6';
import http from 'k6/http';
import ws from 'k6/ws';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.3/index.js';

export const options = {
    thresholds: {
        checks: ['rate == 1.00'],
    },
};

function registerClient() {
    const response = http.post(
        'http://gyro-test:8000/api/register/',
        JSON.stringify({ user_id: 1, component: "test" }),
        { headers: { "Content-Type": "application/json" } }
    );

    return {
        response: response,
        websocket: {
            url: response.json().url,
            id: response.json().id
        }
    };
}

export default function testWebsocketEndpoints() {
    describe('Register a client and connect to WebSocket', () => {
        const clientInfo = registerClient();

        expect(clientInfo.response.status, 'response status').to.equal(200);
        expect(clientInfo.response).to.have.validJsonBody();
        expect(clientInfo.websocket.url, 'url').to.be.a('string');

        const res = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            socket.close();
        });

        check(res, { 'status is 101': (r) => r && r.status === 101 });
    });

    describe('Websocket endpoint is not available after unregister', () => {
        const clientInfo = registerClient();
        const response = http.del(
            `http://gyro-test:8000/api/register/${clientInfo.websocket.id}`,
            { headers: { "Content-Type": "application/json" }}
        );

        check(response, { 'status is 204': (r) => r && r.status === 204 });

        const res = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            fail('websocket connection has not failed');
        });

        check(res, { 'status is 404': (r) => r && r.status === 404 });
    })

    describe('Websocket replies to ping messages', () => {
        let wasPonged = false;

        const clientInfo = registerClient();
        const res = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            socket.on('pong', () => {
                wasPonged = true

                socket.close();
            });

            socket.ping();
        });

        check(wasPonged, { 'server replied to ping': () => wasPonged === true });
    });

    describe('Alerts are published to websocket clients', () => {
        const clientInfo = registerClient();

        let wasNotified = false;
        let notificationMessage = "";

        const res = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            socket.on('message', (message) => {
                console.log(message);

                notificationMessage = message;
                wasNotified = true;

                socket.close();
            });

            http.post(
                'http://gyro-test:8000/api/alerts/',
                JSON.stringify({
                    "name": "Test alert",
                    "component": "CIS"
                }),
                { headers: { "Content-Type": "application/json" } }
            );
        });

        check(wasNotified, { 
            'server published alert notification': () => wasNotified === true,
            'notification message is valid': () => notificationMessage === "Danger Will Robinson Danger !"
        });        
    })
}
