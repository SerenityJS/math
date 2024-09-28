import test from 'ava'

import { AABB, Vector3f } from '../index.js'

test('sum from native', (t) => {
  const x = new AABB(new Vector3f(0,0,0), new Vector3f(1,1,1))
  t.assert(x.min.x == 0)
})
