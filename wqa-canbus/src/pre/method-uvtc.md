# UV TC Method

## Measurement 


check GP1_status (on or off)
 if GP1_status = off then GP1_start

check UV_lamp_status (on or off)
 if UV_lamp_status = off then UV_lamp_on

wait measurement_delay_time

wait reactor_delay_time

start signal_record

 signal_delay = (signal_count_time / signal_count)
 x = 1
 for x to signal_count	
	get signal_NDIR1[x] 
	wait signal_delay

signal_mean_value = (summe von signal_NDIR1 / signal count)

end




GP1_start
 set Dout(GP1) = on
 set GP1_status = on

UV_lamp_start
 set Dout(UV_lamp) = on
 set UV_lamp_status = on

