import testWebsocketEndpoints from "./functional/ws-endpoints.js";
import { Httpx } from 'https://jslib.k6.io/httpx/0.1.0/index.js';
import { describe } from 'https://jslib.k6.io/k6chaijs/4.3.4.3/index.js';
import testAlertEndpoints from "./functional/alert-endpoints.js";

const session = new Httpx({
    baseURL: 'http://gyro-test:8000',
    headers: {
      'User-Agent': 'K6 test runner',
      'Content-Type': 'application/json',
    },
    timeout: 20000,
  });

export default function () {
    describe(
        'Alert endpoints can be used to create and list alerts', 
        () => testAlertEndpoints(session)
    );

    describe(
        'Users can register/unregister from notifications and connect to WebSocket', 
        () => testWebsocketEndpoints(session)
    );
}
