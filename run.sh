#!/bin/bash

ID="$1"
shift
CALLBACK="$1"
shift

PHOTO_DIR="${DATA_DIR}/${ID}"
WORKDIR="${PHOTO_DIR}_workdir"
MODEL_DEST="${RES_DIR}/${ID}.tar.gz"

LOG_FILE="/logs/job/${ID}"
STEP_LOG_FILE="/logs/job/${ID}_step"
GO_NOGO_FILE="/logs/job/${ID}_gonogo"

echo 1 > "$GO_NOGO_FILE" # Activating process

mkdir -p "$PHOTO_DIR"

echo "Start job $ID - $(date)" >"$LOG_FILE"

for i in "$@"; do
  wget -a "$LOG_FILE" "$i" -P "$PHOTO_DIR"
done

echo "Downloaded images:" &>>"$LOG_FILE"
ls "$PHOTO_DIR" &>>"$LOG_FILE"

FINAL_STEP=17

for step in $(seq 0 $FINAL_STEP)
do
  echo -n $step > "$STEP_LOG_FILE" # Logging current step. -n to avoid new line
  # Check if process has been stopped
  while grep -q "0" "$GO_NOGO_FILE"
  do
    echo "Process stopped at step $step. Waiting for activation." &>>"$LOG_FILE"
    sleep 10
  done
  # Process is active
  mvgmvs "$PHOTO_DIR" "$WORKDIR" &>>"$LOG_FILE" --steps $step # Run the next mvgmvs step
done

cd "${WORKDIR}/mvs/"
tar -zcvf "$MODEL_DEST" *.ply *.png &>>"$LOG_FILE"

rm -rf "$WORKDIR"
rm -rf "$PHOTO_DIR"

echo "Notifying URL: $CALLBACK" &>>"$LOG_FILE"
curl "$CALLBACK" &>>"$LOG_FILE"