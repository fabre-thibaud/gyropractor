import { check, fail } from 'k6';
import ws from 'k6/ws';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.3/index.js';

export const options = {
    thresholds: {
        checks: ['rate == 1.00'],
    },
};

export default function testWebsocketEndpoints(session) {
    function registerClient() {
        const response = session.post(
            '/api/register/',
            JSON.stringify({ user_id: 1, component: "test" })
        );
    
        return {
            response: response,
            websocket: {
                url: response.json().url,
                id: response.json().id
            }
        };
    }

    describe('Register a client and connect to WebSocket', () => {
        const clientInfo = registerClient();

        expect(clientInfo.response.status, 'response status').to.equal(200);
        expect(clientInfo.response).to.have.validJsonBody();
        expect(clientInfo.websocket.url, 'url').to.be.a('string');

        const response = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            socket.close();
        });

        expect(response.status, 'websocket connection status').to.equal(101);
    });

    describe('Websocket endpoint is not available after unregister', () => {
        const clientInfo = registerClient();

        let response = session.delete(`/api/register/${clientInfo.websocket.id}`);

        expect(response.status, 'delete registration response status').to.equal(204);

        response = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            fail('websocket connection has not failed');
        });

        expect(response.status, 'websocket response status').to.equal(404);
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

        expect(wasPonged, 'pong received').to.equal(true);
    });

    describe('Alerts are published to websocket clients', () => {
        const clientInfo = registerClient();

        let wasNotified = false;
        let notificationMessage = "";

        const response = ws.connect(clientInfo.websocket.url, {}, function (socket) {
            socket.on('message', (message) => {
                notificationMessage = message;
                wasNotified = true;

                socket.close();
            });

            session.post(
                '/api/alerts/',
                JSON.stringify({
                    "name": "Test alert",
                    "component": "CIS"
                })
            );
        });

        expect(wasNotified, 'notification received').to.equal(true);
        expect(notificationMessage, 'notification message').to.equal("Danger Will Robinson Danger !");
    })
}
