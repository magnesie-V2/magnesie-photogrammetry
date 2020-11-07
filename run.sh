
INPUT="$1"
WORKDIR="${INPUT}_workdir"
RES="/res/$(basename $INPUT)"

python3 openMVS/MvgMvsPipeline.py "/datasets/$INPUT" "$WORKDIR"

mkdir "$RES"
cp "${WORKDIR}/mvs/*.ply" "$RES/"

rm -rf "$WORKDIR"