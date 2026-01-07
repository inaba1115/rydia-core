import numpy as np
import sounddevice as sd  # type: ignore

import rydia

sr = 48000.0
sec = 5.0

y = np.zeros(int(sr * sec))
white_osc = rydia.WhiteNoise()

for n in range(len(y)):
    y[n] = white_osc.process()

sd.play(y, samplerate=sr, blocking=True)
