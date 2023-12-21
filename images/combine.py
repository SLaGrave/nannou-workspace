import imageio
import glob

images = []
for filename in glob.glob("schotter4_frames/*.png"):
    images.append(imageio.imread(filename))
imageio.mimsave('./schotter4.gif', images)