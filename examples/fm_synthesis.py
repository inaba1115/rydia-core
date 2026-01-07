import numpy as np
import sounddevice as sd  # type: ignore

import rydia

sr = 48000.0
sec = 5.0
car_freq = 456.0
mod_freq = 321.0
mod_index = 252.0

y = np.zeros(int(sr * sec))
car_osc = rydia.SinOsc(sr)
mod_osc = rydia.SinOsc(sr)

for n in range(len(y)):
    mod = mod_osc.process(mod_freq) * mod_index
    y[n] = car_osc.process(car_freq + mod)

sd.play(y, samplerate=sr, blocking=True)
