10
01#
01#OIB
1
10

#q1: Gehe nach recht bis # -> q2
1 B -
1 # 2 # R
1 0 1 0 R
1 1 1 1 R
1 O 1 O R
1 I 1 I R

#q2: (#) Gehe nach rechts bis B -> q3
2 B 3 B L
2 # -
2 0 2 0 R
2 1 2 1 R
2 O -
2 I -

#q3: (check 0|1) Wenn # cleanup, ansonsten lösche und wenn 0 -> q4 oder 1 -> q5
3 B -
3 # 9 B L
3 0 4 B L
3 1 5 B L
3 O -
3 I -

#q4: (0) Gehe nach links bis # -> q6
4 B -
4 # 6 # L
4 0 4 0 L
4 1 4 1 L
4 O -
4 I -

#q5: (1) Gehe nach links bis # -> q7
5 B -
5 # 7 # L
5 0 5 0 L
5 1 5 1 L
5 O -
5 I -

#q6: (0, #) Wenn O oder I gehe nach links, ansonsten O/I an bei nächstem Element -> q1
6 B 1 O R
6 # -
6 0 1 O R
6 1 1 I R
6 O 6 O L
6 I 6 I L

#q7: (1, #) Wenn O oder I gehe nach links, bei 0 schreibe 1 (-> q1), bei 1 schreibe O -> q8
7 B 1 O R
7 # -
7 0 1 I R
7 1 8 O L
7 O 7 O L
7 I 7 I L

#q8: (übertrag) Behebe übertrag (überschreibe 1 mit 0 bis erste 0 gefunden) -> q1
8 B 1 1 R
8 # -
8 0 1 1 R
8 1 8 0 L
8 O -
8 I -

#q9: (cleanup) Gehe nach links bis B unde ersetze O/I -> q10
9 B 10 B R
9 # -
9 0 9 0 L
9 1 9 1 L
9 O 9 0 L
9 I 9 1 L
