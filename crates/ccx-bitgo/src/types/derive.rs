use macro_rules_attribute::attribute_alias;

attribute_alias! {
    #[apply(Request)] =
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize, Debug, Clone, bon::Builder)]
        #[builder(on(_, into))]
        #[serde(rename_all = "camelCase")];

    #[apply(Response)] =
        #[derive(serde::Deserialize, Debug, Clone)]
        #[serde(rename_all = "camelCase")];
}
