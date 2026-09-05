// expected: chalks: smoothness must be in [0, 1]
#import "@preview/chalks:0.1.1": raw-stroke
#raw-stroke(((0, 0), (10, 10)), style: (smoothness: 1.5))
