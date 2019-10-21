table! {
    atendimentos (id) {
        id -> Integer,
        id_professor -> Integer,
        dia -> Varchar,
        hora_inicio -> Time,
        hora_fim -> Time,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    atividades_administrativas (id) {
        id -> Integer,
        id_professor -> Integer,
        atividade -> Varchar,
        numero_portaria -> Integer,
        ano_portaria -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    atividades_extensao (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    atividades_pesquisa (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    aulas (id) {
        id -> Integer,
        id_professor -> Integer,
        curso_id -> Nullable<Integer>,
        componente_curricular_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    ciclos_letivos (id) {
        id -> Integer,
        ano -> Nullable<Integer>,
        semestre -> Nullable<Integer>,
        nivel_ensino_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    componentes_curriculares (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        curso_id -> Nullable<Integer>,
        ch_semanal -> Float,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    cursos (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        nivel_ensino_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    niveis_ensino (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    professors (id) {
        id -> Integer,
        user_id -> Integer,
        nome -> Nullable<Varchar>,
        sexo -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        area_conhecimento -> Nullable<Varchar>,
        categoria -> Nullable<Varchar>,
        regime_trabalho -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    projetos_ensino (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    reunioes_formacao_docente (id) {
        id -> Integer,
        id_professor -> Integer,
        dia -> Varchar,
        hora_inicio -> Time,
        hora_fim -> Time,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        login -> Varchar,
        password_hash -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

joinable!(atendimentos -> professors (id_professor));
joinable!(atividades_administrativas -> professors (id_professor));
joinable!(atividades_extensao -> professors (id_professor));
joinable!(atividades_pesquisa -> professors (id_professor));
joinable!(aulas -> componentes_curriculares (componente_curricular_id));
joinable!(aulas -> cursos (curso_id));
joinable!(aulas -> professors (id_professor));
joinable!(ciclos_letivos -> niveis_ensino (nivel_ensino_id));
joinable!(componentes_curriculares -> cursos (curso_id));
joinable!(cursos -> niveis_ensino (nivel_ensino_id));
joinable!(professors -> users (user_id));
joinable!(projetos_ensino -> professors (id_professor));
joinable!(reunioes_formacao_docente -> professors (id_professor));

allow_tables_to_appear_in_same_query!(
    atendimentos,
    atividades_administrativas,
    atividades_extensao,
    atividades_pesquisa,
    aulas,
    ciclos_letivos,
    componentes_curriculares,
    cursos,
    niveis_ensino,
    professors,
    projetos_ensino,
    reunioes_formacao_docente,
    users,
);
