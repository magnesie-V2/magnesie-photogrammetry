
INPUT="$1"
WORKDIR="${INPUT}_workdir"
RES="/res/$(basename $INPUT).ply"

python3 openMVS/MvgMvsPipeline.py "/datasets/$INPUT" "$WORKDIR"

cp "${WORKDIR}/mvs/scene_dense_mesh_refine_texture.ply" "$RES"

rm -rf "$WORKDIR"