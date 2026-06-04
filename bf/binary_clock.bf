Read 3 bytes (h,m,s) from input, output each as dots
Tape: [h][m][s][dot][nl]

,>,>,> read h,m,s (cells 0,1,2)
<<<   back to cell0

Set cell3 = '.' (46):
>>>++++++++++++++++++++++++++++++++++++++++++++++<<<

Set cell4 = '\n' (10):
>>>>++++++++++<<<<

Output h:
[->>>.<<<]   output cell3 ('.') h times
>>>>.<<<<    output '\n'

Move m to cell0:
>[->+<]<     cell1 -> cell0, ptr=cell0

Output m:
[->>>.<<<]
>>>>.<<<<

Move s to cell1 then to cell0:
>>[<+>-]<    cell2 -> cell1, ptr=cell1
[<+>-]<      cell1 -> cell0, ptr=cell0

Output s:
[->>>.<<<]
>>>>.<<<<
