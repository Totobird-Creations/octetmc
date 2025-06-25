use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use std::collections::HashMap;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;
use convert_case::{ Case, Casing };


pub async fn registries(generated_dir : &Path, target_dir : &Path) {
    fs::create_dir_all(target_dir).await.unwrap();

    let registries_file = generated_dir.join("reports/registries.json");
    let registries      = read_json::<_, Registries>(File::open(registries_file).unwrap()).unwrap();

    {
        println!("Generating entity types");
        let mut target = File::create(target_dir.join("entity_type.rs")).unwrap();
        writeln!(target, "impl EntityType {{").unwrap();
        for (id, RegistryEntry { protocol_id },) in registries.entity_types.entries {
            let name  = id.strip_prefix("minecraft:").unwrap();
            let ident = name.to_case(Case::UpperSnake);
            writeln!(target, "    /// Vanilla `{id}` entity type.").unwrap();
            writeln!(target, "    pub const {ident} : Self = Self {{").unwrap();
            writeln!(target, "        id   : {protocol_id},").unwrap();
            writeln!(target, "        name : {name:?},").unwrap();
            writeln!(target, "    }};").unwrap();
        }
        writeln!(target, "}}").unwrap();
    }

}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct Registries {
    #[expect(dead_code)]
    #[serde(rename = "minecraft:activity")]
    activities                              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:attribute")]
    attributes                              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:block")]
    blocks                                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:block_entity_type")]
    block_entity_types                      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:block_predicate_type")]
    block_predicate_types                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:block_type")]
    block_types                             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:chunk_status")]
    chunk_statuses                          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:command_argument_type")]
    command_argument_types                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:consume_effect_type")]
    consume_effect_type                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:creative_mode_tab")]
    creative_mode_tabs                      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:custom_stat")]
    custom_stats                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:data_component_predicate_type")]
    data_component_predicate_types          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:data_component_type")]
    data_component_types                    : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:decorated_pot_pattern")]
    decorated_pot_patterns                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:dialog_action_type")]
    dialog_action_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:dialog_body_type")]
    dialog_body_types                       : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:dialog_type")]
    dialog_types                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_effect_component_type")]
    enchantment_effect_component_types      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_entity_effect_type")]
    enchantment_entity_effect_types         : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_level_based_value_type")]
    enchantment_level_based_value_types     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_location_based_effect_type")]
    enchantment_location_based_effect_types : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_provider_type")]
    enchantment_provider_types              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:enchantment_value_effect_type")]
    enchantment_value_effect_types          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:entity_sub_predicate_type")]
    entity_sub_predicate_types              : IgnoredAny,
    #[serde(rename = "minecraft:entity_type")]
    entity_types                            : Registry,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:float_provider_type")]
    float_provider_types                    : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:fluid")]
    fluids                                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:game_event")]
    game_events                             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:height_provider_type")]
    height_provider_types                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:input_control_type")]
    input_control_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:int_provider_type")]
    int_provider_types                      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:item")]
    items                                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_condition_type")]
    loot_condition_types                    : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_function_type")]
    loot_function_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_nbt_provider_type")]
    loot_nbt_provider_types                 : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_number_provider_type")]
    loot_number_provider_types              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_pool_entry_type")]
    loot_pool_entry_types                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:loot_score_provider_type")]
    loot_score_provider_types               : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:map_decoration_type")]
    map_decoration_types                    : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:memory_module_type")]
    memory_module_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:menu")]
    menus                                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:mob_effect")]
    mob_effects                             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:number_format_type")]
    number_format_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:particle_type")]
    particle_types                          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:point_of_interest_type")]
    point_of_interest_types                 : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:pos_rule_test")]
    pos_rule_tests                          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:position_source_type")]
    position_source_types                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:potion")]
    potions                                 : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:recipe_book_category")]
    recipe_book_categories                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:recipe_display")]
    recipe_displays                         : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:recipe_serializer")]
    recipe_serialisers                      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:recipe_type")]
    recipe_types                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:rule_block_entity_modifier")]
    rule_block_entity_modifiers             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:rule_test")]
    rule_tests                              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:schedule")]
    schedules                               : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:sensor_type")]
    sensor_types                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:slot_display")]
    slot_displays                           : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:sound_event")]
    sound_events                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:spawn_condition_type")]
    spawn_condition_types                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:stat_type")]
    stat_types                              : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:test_environment_definition_type")]
    test_environment_definition_types       : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:test_function")]
    test_functions                          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:test_instance_type")]
    test_instance_types                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:ticket_type")]
    ticket_types                            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:trigger_type")]
    trigger_types                           : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:villager_profession")]
    villager_professions                    : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:villager_type")]
    villager_types                          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(flatten)]
    worldgen                                : WorldgenRegistries
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct WorldgenRegistries {
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/biome_source")]
    biome_source             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/block_state_provider_type")]
    biome_state_provider_type: IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/carver")]
    carvers                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/chunk_generator")]
    chunk_generators         : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/density_function_type")]
    density_function_types   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/feature")]
    features                 : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/feature_size_type")]
    feature_size_types       : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/foliage_placer_type")]
    foliage_placer_types     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/material_condition")]
    material_condition       : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/material_rule")]
    material_rule            : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/placement_modifier_type")]
    placement_modifier_types : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/pool_alias_binding")]
    pool_alias_bindings      : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/root_placer_type")]
    root_placer_types        : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/structure_piece")]
    structure_pieces         : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/structure_placement")]
    structure_placements     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/structure_pool_element")]
    structure_pool_elements  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/structure_processor")]
    structure_processors     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/structure_type")]
    structure_types          : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/tree_decorator_type")]
    tree_decorator_types     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minecraft:worldgen/trunk_placer_type")]
    trunk_placer_types       : IgnoredAny
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    #[expect(dead_code)]
    protocol_id : u32,
    #[expect(dead_code)]
    default     : Option<String>,
    entries     : HashMap<String, RegistryEntry>
}


#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct RegistryEntry {
    protocol_id : u32
}
