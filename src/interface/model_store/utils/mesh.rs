


struct Mesh {


    mesh_part_id: u32,
    index_id: String,
    position_id: String,
    normal_id: Option<String>,
    texture_id: Option<String>,


    item_id: Vec<u32>,

    item_material_id: Vec<u32>,
    item_material_default_id: Vec<u32>,
    item_material_override_id: Vec<u32>,

    item_start: Vec<u32>,
    item_count: Vec<u32>,

    item_hidden: Vec<bool>,
    item_transform_id: Vec<u32>,
    item_bounding_boxes: Vec<Box>,

    materials: HashMap<u32, Color, fnv_hasher::U32Hasher>,
    transforms: HashMap<u32, Option<TransfomationMatrix>, fnv_hasher::U32Hasher>,

}