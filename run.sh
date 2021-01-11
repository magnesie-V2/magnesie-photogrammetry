#!/bin/bash

ID="$1"
shift
CALLBACK="$1"
shift

PHOTO_DIR="${DATA_DIR}/${ID}"
WORKDIR="${PHOTO_DIR}_workdir"
MODEL_DEST="${RES_DIR}/${ID}.tar.gz"

LOG_FILE="/logs/job/${ID}"

mkdir -p "$PHOTO_DIR"

echo "Start job $ID - $(date)" >"$LOG_FILE"

for i in "$@"; do
  wget -a "$LOG_FILE" "$i" -P "$PHOTO_DIR"
done

echo "Downloaded images:" &>>"$LOG_FILE"
ls "$PHOTO_DIR" &>>"$LOG_FILE"
python3 /openMVS/MvgMvsPipeline.py "$PHOTO_DIR" "$WORKDIR" &>>"$LOG_FILE"

#mkdir -p "$MODEL_DIR"
#cp "${WORKDIR}/mvs/"*".ply" "$MODEL_DIR/" &>>"$LOG_FILE"
#cp "${WORKDIR}/mvs/"*".png" "$MODEL_DIR/" &>>"$LOG_FILE"
cd "${WORKDIR}/mvs/"
tar -zcvf "$MODEL_DEST" *.ply *.png &>>"$LOG_FILE"

rm -rf "$WORKDIR"
rm -rf "$PHOTO_DIR"

echo "Notifying URL: $CALLBACK" &>>"$LOG_FILE"
curl "$CALLBACK" &>>"$LOG_FILE"
