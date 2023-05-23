#!/bin/bash
# conda init "$(basename "${SHELL}")"
# conda create -n py310-whisper python=3.10 -y
conda activate py310-whisper
pip install ane_transformers
pip install openai-whisper
pip install coremltools
python3 -m pip install urllib3==1.26.6
/opt/thalamus/models/generate-coreml-model.sh tiny
/opt/thalamus/models/generate-coreml-model.sh base
/opt/thalamus/models/generate-coreml-model.sh medium
/opt/thalamus/models/generate-coreml-model.sh large