import http from 'k6/http';
import { check, group } from 'k6';

export const options = {
    scenarios: {
        "health-ramp": {
            executor: 'constant-vus',
            duration: '60s',
            vus: 10
        },
    }
};

export default function () {
    group('GET /health', () => {
        const res = http.get('http://gyro-test:8000/health');

        check(res, { 'status was 200': (r) => r.status == 200 });
    });

    group('POST /api/alerts', () => {
        const res = http.post(
            'http://gyro-test:8000/api/alerts/',
            JSON.stringify({
                "name": "Test alert",
                "component": "CIS"
            }),
            { headers: { "Content-Type": "application/json" } }
        );

        check(res, { 'status was 200': (r) => r.status == 200 });
    });
}
