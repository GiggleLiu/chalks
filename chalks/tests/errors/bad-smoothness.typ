// expected: chalks: smoothness must be in [0, 1]
#import "../../lib.typ": raw-stroke
#raw-stroke(((0, 0), (10, 10)), style: (smoothness: 1.5))
