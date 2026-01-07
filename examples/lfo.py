import numpy as np
import sounddevice as sd  # type: ignore

import rydia

sr = 48000.0
sec = 5.0
osc_freq = 440.0
lfo_freq = 5.0

y = np.zeros(int(sr * sec))
osc = rydia.SinOsc(sr)
lfo = rydia.Lfo(sr, 0)


for n in range(len(y)):
    v, _ = lfo.process(lfo_freq)
    y[n] = osc.process(osc_freq + v * 30.0)

sd.play(y, samplerate=sr, blocking=True)
