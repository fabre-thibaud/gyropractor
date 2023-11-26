import http from 'k6/http';

export const options = {
    scenarios: {
        health: {
            executor: 'ramping-vus',
            startVUs: 3,
            stages: [
                { target: 20, duration: '30s' },
                { target: 100, duration: '0' },
                { target: 100, duration: '10m' },
            ],
        },
    }
};

export default function () {
  http.get('http://gyro-test:8000/health');
}
