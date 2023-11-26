import testWebsocketEndpoints from "./ws-test.js";
import { describe, expect } from 'https://jslib.k6.io/k6chaijs/4.3.4.3/index.js';

export default function () {
    describe(
        'Users can register/unregister from notifications and connect to WebSocket', 
        testWebsocketEndpoints
    );
}
