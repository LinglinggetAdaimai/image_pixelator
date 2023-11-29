import numpy as np
import cv2
import multiprocessing

def pixelate_block(image_block):
    # Calculate the average color of the block
    average_color = np.mean(image_block, axis=(0, 1))
    
    # Replace all pixels in the block with the average color
    image_block[:, :] = average_color

def pixelate_image(image, block_size):
    height, width = image.shape[:2]
    
    for y in range(0, height, block_size):
        for x in range(0, width, block_size):
            # Process each block of the image
            block = image[y:y + block_size, x:x + block_size]
            pixelate_block(block)

# Example usage
input_image_path = '../images/input_img.jpg'
output_image_path = '../images/pixelated_image.jpg'
block_size = 10

# Read the input image
image = cv2.imread(input_image_path)

# Divide the image into smaller blocks and process them in parallel
with multiprocessing.Pool() as pool:
    # Split the image into blocks and process each block in parallel
    blocks = [image[y:y + block_size, x:x + block_size] for y in range(0, image.shape[0], block_size) for x in range(0, image.shape[1], block_size)]
    pool.map(pixelate_block, blocks)

# Reconstruct the image from pixelated blocks
reconstructed_image = np.concatenate([np.concatenate(blocks_row, axis=1) for blocks_row in np.array_split(blocks, image.shape[0] // block_size)], axis=0)

# Save the final pixelated image
cv2.imwrite(output_image_path, reconstructed_image)

# Display the original and pixelated images
cv2.imshow('Original Image', image)
cv2.imshow('Pixelated Image', reconstructed_image)
cv2.waitKey(0)
cv2.destroyAllWindows()