import { check, fail } from 'k6';
import ws from 'k6/ws';
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.3/index.js';

export const options = {
    thresholds: {
        checks: ['rate == 1.00'],
    },
};

export default function testAlertEndpoints(session) {
    describe('Create a new alert', () => {
        const minDate = new Date();

        const response = session.post(
            '/api/alerts/',
            JSON.stringify({
                "name": "Test alert",
                "component": "CIS"
            })
        );

        const maxDate = new Date();

        expect(response.status, 'response status').to.equal(200);
        expect(response).to.have.validJsonBody();

        const alert = JSON.parse(response.body);
        const alertDate = new Date(Date.parse(alert.created_at));

        expect(alert.name, 'name').to.equal("Test alert");
        expect(alert.component, 'component').to.equal("CIS");
        expect(alert.checked, 'checked').to.equal(false);
        expect(minDate <= alertDate, "min creation date").to.equal(true);
        expect(maxDate >= alertDate, "max creation date").to.equal(true);
        expect(alert.checked_at, "checked date").to.equal(null);
    });

    describe('Alerts list is sorted by most recent alerts', () => {
        let response = session.post(
            '/api/alerts/',
            JSON.stringify({
                "name": "Test alert",
                "component": "CIS"
            })
        );

        expect(response.status, 'create response status').to.equal(200);
        expect(response).to.have.validJsonBody();

        const alert = JSON.parse(response.body);

        response = session.get('/api/alerts/');

        expect(response.status, 'list response status').to.equal(200);
        expect(response).to.have.validJsonBody();

        const alerts = JSON.parse(response.body);

        expect(JSON.stringify(alerts[0]), 'alerts[0]').to.equal(JSON.stringify(alert));
    })

    function getAlerts(page, pageSize) {
        const response = session.get(`/api/alerts/?page=${page}&page_size=${pageSize}`);

        expect(response.status, 'list response status').to.equal(200);
        expect(response).to.have.validJsonBody();

        return JSON.parse(response.body);
    }

    describe('Alerts list can be paginated by page number and items per page', () => {
        for (let i = 0; i < 20; i++) {
            session.post(
                '/api/alerts/',
                JSON.stringify({
                    "name": "Test alert",
                    "component": "CIS"
                })
            );
        }

        const alerts_1_10 = getAlerts(1, 10);
        const alerts_2_10 = getAlerts(2, 10);
        const alerts_1_20 = getAlerts(1, 20);

        expect(alerts_1_10.length, 'alerts_1_10.length').to.equal(10);
        expect(alerts_2_10.length, 'alerts_2_10.length').to.equal(10);
        expect(alerts_1_20.length, 'alerts_2_10.length').to.equal(20);

        expect(JSON.stringify(alerts_1_20.slice(0, 10)), 'alerts_1_20.slice(0, 10)').to.equal(JSON.stringify(alerts_1_10));
        expect(JSON.stringify(alerts_1_20.slice(10, 20)), 'alerts_1_20.slice(10, 20)').to.equal(JSON.stringify(alerts_2_10));
    })

    describe('Alerts list cannot be paginated using negative page numbers', () => {
        let response;
        
        response = session.get(`/api/alerts/?page=-1&page_size=10`);

        expect(response.status, 'list response status').to.equal(400);

        response = session.get(`/api/alerts/?page=1&page_size=-10`);

        expect(response.status, 'list response status').to.equal(400);
    })
}
