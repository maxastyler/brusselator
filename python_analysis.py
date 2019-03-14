import numpy as np
import pickle
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from matplotlib.colors import LogNorm

with open("a_1_b_1.9_x_0.9_y_1.8_g_0_0_0_0_dt_0.001.brus", "rb") as f:
    pos_1 = np.array(pickle.load(f))
with open("a_1_b_1.9_x_0.9_y_1.8_g_0.01_0_0_0.01_dt_0.001.brus", "rb") as f:
    pos_2 = np.array(pickle.load(f))

#plt.plot(data[1, :, 0], data[1, :, 1])

fig, ax = plt.subplots(2)
limits = [[0.6, 1.4], [1.6, 2.5]]

#Create 2d Histogram
#data_1,_,_ = np.histogram2d(pos_1[:, 0, 0], pos_1[:, 0, 1], bins = 200, range=limits, density=True)
#data_2,_,_ = np.histogram2d(pos_2[:, 0, 0], pos_2[:, 0, 1], bins = 200, range=limits, density=True)
data_1,_,_ = np.histogram2d(pos_1[:, 0, 0], pos_1[:, 0, 1], bins = 200, range=limits)
data_2,_,_ = np.histogram2d(pos_2[:, 0, 0], pos_2[:, 0, 1], bins = 200, range=limits)

#Smooth with filter
im_1 = ax[0].imshow(data_1 + 0.01, interpolation = 'gaussian', origin = 'lower', norm=LogNorm(vmin=0.01, vmax=1000))
im_2 = ax[1].imshow(data_2 + 0.01, interpolation = 'gaussian', origin = 'lower', norm=LogNorm(vmin=0.01, vmax=1000))

#Define animation. 
def animate(i) :
    data_1,_,_ = np.histogram2d(pos_1[:, i, 0], pos_1[:, i, 1], bins = 200, range=limits)
    data_2,_,_ = np.histogram2d(pos_2[:, i, 0], pos_2[:, i, 1], bins = 200, range=limits)
    im_1.set_data(data_1 + 0.01)
    im_2.set_data(data_2 + 0.01)

ani = animation.FuncAnimation(fig, animate, np.arange(0,490),
                          interval = 100, blit = False)

plt.show()
