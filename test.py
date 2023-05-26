# Copyright 2018 The TensorFlow Authors. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ==============================================================================
"""label_image for tflite."""

import argparse
import time
import os
import numpy as np
from PIL import Image
import tensorflow as tf
import shutil
import string
import random

def get_list_dir():
    list_dir = os.listdir()
    return list_dir

def load_labels(filename):
    with open(filename, 'r') as f:
        return [line.strip() for line in f.readlines()]


if __name__ == '__main__':
  

    ext_delegate = None
    ext_delegate_options = {}

    interpreter = tf.lite.Interpreter(
        model_path="/home/kal/Documents/PixelCoda/Thalamus/packages/ocnn/birds/birds.tflite",
        experimental_delegates=ext_delegate,
        num_threads=1)
    interpreter.allocate_tensors()

    input_details = interpreter.get_input_details()
    output_details = interpreter.get_output_details()

    # check the type of the input tensor
    floating_model = input_details[0]['dtype'] == np.float32

    # NxHxWxC, H:1, W:2
    height = input_details[0]['shape'][1]
    width = input_details[0]['shape'][2]
    labels = load_labels("/home/kal/Documents/PixelCoda/Thalamus/packages/ocnn/birds/birds.txt")



    try:
        image_path = "/home/kal/Documents/PixelCoda/Thalamus/test.jpg"

        img = Image.open(image_path).resize((width, height))

        # add N dim
        input_data = np.expand_dims(img, axis=0)

        if floating_model:
            input_data = (np.float32(input_data) - 127.5) / 127.5

        interpreter.set_tensor(input_details[0]['index'], input_data)

        start_time = time.time()
        interpreter.invoke()
        stop_time = time.time()

        output_data = interpreter.get_tensor(output_details[0]['index'])
        results = np.squeeze(output_data)

        top_k = results.argsort()[-5:][::-1]

        for i in top_k:
            if float(results[i]) > 0:
                print('{:08.6f}: {}'.format(float(results[i]), labels[i]))
            


        

        print('time: {:.3f}ms'.format((stop_time - start_time) * 1000))

    except:
        print('')

    os.chdir('../')


