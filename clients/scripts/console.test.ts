import {identity} from "@ego-js/utils";

describe('console', () => {
    test('console', async () => {
        console.log(identity().getPrincipal().toText())
    });
});