## Input  [[1, 2], [2, 3], ...]
## Output [ 5    ,  8    , ...]

Node #1
===========
IN:1 -> 1
------
# Double the first value
MOV 1>, ACC
ADD ACC
MOV ACC, >1
------
1 -> #3:1
===========

Node #2
===========
IN:2 -> 1
-----------
# Increment the second value
MOV 1>, ACC
ADD 1
MOV ACC, >1
-----------
1 -> #3:2
===========

Node #3
=======
# Possible to repeat the same source (for readability)
#1:1 -> 1, #2:1 -> 2
---------
MOV 1>, ACC
ADD 2> # Sum the values
MOVE ACC, >1
------------
1 -> OUT:1
=========
