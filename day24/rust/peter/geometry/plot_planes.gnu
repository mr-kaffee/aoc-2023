# output to PNG file day24_geometry.png
set terminal pngcairo enhanced font 'Verdana,10' size 800,600
set output 'day24_geometry.png'

# B': [0, 4, -8] + u * [1, -2, 0], 
# C': [1, 12, 4] + u * [0, -3, -2]
# Rock': [5, 0, -20] + u * [-1, 0, 4]
# Hit times: A at 5, B at 3, C at 4

set view 25, 355

set xlabel "x"
set ylabel "y"
set zlabel "z"

set parametric

set urange [0:1]
set vrange [0:1]

splot \
    5-5*u,0,-20+4*5*u with line lw 1 lc rgb "red" title "Rock'", \
    5-0,0,-20+4*0 with points pt 7 lc rgb "red" notitle, \
    5-1,0,-20+4*1 with points pt 7 lc rgb "red" notitle, \
    5-2,0,-20+4*2 with points pt 7 lc rgb "red" notitle, \
    5-5,0,-20+4*5 with points pt 2 ps 1.5 lc rgb "magenta" notitle, \
    1,12-3*4*u,4-2*4*u with line lw 1 lc rgb "green" title "C'", \
    v,12*v,4*v with lines lw .2 lc rgb "green" notitle, \
    v,9*v,2*v with lines lw .2 lc rgb "green" notitle, \
    1,12-3*0,4-2*0 with points pt 5 ps 1.2 lc rgb "green" notitle, \
    1,12-3*1,4-2*1 with points pt 5 ps 1.2 lc rgb "green" notitle, \
    1,12-3*2,4-2*2 with points pt 7 lc rgb "green" notitle, \
    1,12-3*3,4-2*3 with points pt 7 lc rgb "green" notitle, \
    1,12-3*4,4-2*4 with points pt 2 ps 1.5 lc rgb "green" notitle, \
    -1+3*u,6-2*3*u,-8 with line lw 1 lc rgb "blue" title "B'", \
    -v,6*v,-8*v with lines lw .2 lc rgb "blue" notitle, \
    0,4*v,-8*v with lines lw .2 lc rgb "blue" notitle, \
    -1+0,6-2*0,-8 with points pt 5 ps 1.2 lc rgb "blue" notitle, \
    -1+1,6-2*1,-8 with points pt 5 ps 1.2 lc rgb "blue" notitle, \
    -1+2,6-2*2,-8 with points pt 7 lc rgb "blue" notitle, \
    -1+3,6-2*3,-8 with points pt 2 ps 1.5 lc rgb "blue" notitle


      
