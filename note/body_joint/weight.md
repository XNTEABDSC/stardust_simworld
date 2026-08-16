# weight

[(v_change,b_inertia),...]

A:(0,0),mass:1

B:(2,0),mass:2

C:(0,0),mass:3

J1: A-B 1

A:+2/3 B:-1/3

mass_sum=3 mass_mul=2

J2: B-C 1

C:+2/6 B:-3/6

total_mass=5 mass_mul=6

->

A,C:+1/3

B:-2/3

J1 * 1/2

J2 * 1
