G21         ; Set units to mm
G90         ; Absolute positioning
G1 Z2.54 F2540      ; Move to clearance level

;
; Operation:    0
; Name:         
; Type:         Engrave
; Paths:        1
; Direction:    Conventional
; Cut Depth:    3.175
; Pass Depth:   3.175
; Plunge rate:  127
; Cut rate:     1016
;

; Path 0
; Rapid to initial position
G1 X1.2924 Y-1.0178 F2540
G1 Z0.0000
; plunge
G1 Z-3.1750 F127
; cut
G1 X5.1422 Y-1.0178 F1016
G1 X50.1422 Y-1.1681
G1 X10.2924 Y-1.1681
G1 X100.2924 Y-1.0178
G1 X-150.2924 Y-1.0178
; Retract
G1 Z2.5400 F2540
M2
