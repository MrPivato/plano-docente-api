-- Database: `PDC`

-- CREATE DATABASE IF NOT EXISTS `PDC`;

CREATE TABLE `atendimentos` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `dia` varchar(255) NOT NULL,
  `hora_inicio` time NOT NULL,
  `hora_fim` time NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `atividades_administrativas` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `atividade` varchar(255) NOT NULL,
  `numero_portaria` int(11) NOT NULL,
  `ano_portaria` int(11) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `atividades_extensao` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `titulo` varchar(255) NOT NULL,
  `numero_edital` int(11) NOT NULL,
  `ano_edital` int(11) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `atividades_pesquisa` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `titulo` varchar(255) NOT NULL,
  `numero_edital` int(11) NOT NULL,
  `ano_edital` int(11) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `aulas` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `curso_id` int(11) DEFAULT NULL,
  `componente_curricular_id` int(11) DEFAULT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `ciclos_letivos` (
  `id` int(11) NOT NULL,
  `ano` int(11) DEFAULT NULL,
  `semestre` int(11) DEFAULT NULL,
  `nivel_ensino_id` int(11) DEFAULT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `componentes_curriculares` (
  `id` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  `curso_id` int(11) DEFAULT NULL,
  `ch_semanal` float NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `cursos` (
  `id` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  `nivel_ensino_id` int(11) DEFAULT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `niveis_ensino` (
  `id` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `professors` (
  `id` int(11) NOT NULL,
  `user_id` int(11) NOT NULL,
  `nome` varchar(255) DEFAULT NULL,
  `sexo` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  `area_conhecimento` varchar(255) DEFAULT NULL,
  `categoria` varchar(255) DEFAULT NULL,
  `regime_trabalho` varchar(255) DEFAULT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `projetos_ensino` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `titulo` varchar(255) NOT NULL,
  `numero_edital` int(11) NOT NULL,
  `ano_edital` int(11) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `reunioes_formacao_docente` (
  `id` int(11) NOT NULL,
  `id_professor` int(11) NOT NULL,
  `dia` varchar(255) NOT NULL,
  `hora_inicio` time NOT NULL,
  `hora_fim` time NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE `users` (
  `id` int(11) NOT NULL,
  `login` varchar(255) NOT NULL,
  `password_hash` varchar(255) NOT NULL,
  `role` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

ALTER TABLE `atendimentos`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `atividades_administrativas`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `atividades_extensao`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `atividades_pesquisa`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `aulas`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`),
  ADD KEY `curso_id` (`curso_id`),
  ADD KEY `componente_curricular_id` (`componente_curricular_id`);

ALTER TABLE `ciclos_letivos`
  ADD PRIMARY KEY (`id`),
  ADD KEY `nivel_ensino_id` (`nivel_ensino_id`);

ALTER TABLE `componentes_curriculares`
  ADD PRIMARY KEY (`id`),
  ADD KEY `curso_id` (`curso_id`);

ALTER TABLE `cursos`
  ADD PRIMARY KEY (`id`),
  ADD KEY `nivel_ensino_id` (`nivel_ensino_id`);

ALTER TABLE `niveis_ensino`
  ADD PRIMARY KEY (`id`);

ALTER TABLE `professors`
  ADD PRIMARY KEY (`id`),
  ADD KEY `user_id` (`user_id`);

ALTER TABLE `projetos_ensino`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `reunioes_formacao_docente`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_professor` (`id_professor`);

ALTER TABLE `users`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `login` (`login`);

ALTER TABLE `atendimentos`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `atividades_administrativas`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `atividades_extensao`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `atividades_pesquisa`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `aulas`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `ciclos_letivos`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `componentes_curriculares`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `cursos`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `niveis_ensino`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `professors`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `projetos_ensino`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `reunioes_formacao_docente`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `users`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `atendimentos`
  ADD CONSTRAINT `atendimentos_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);

ALTER TABLE `atividades_administrativas`
  ADD CONSTRAINT `atividades_administrativas_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);

ALTER TABLE `atividades_extensao`
  ADD CONSTRAINT `atividades_extensao_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);

ALTER TABLE `atividades_pesquisa`
  ADD CONSTRAINT `atividades_pesquisa_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);

ALTER TABLE `aulas`
  ADD CONSTRAINT `aulas_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`),
  ADD CONSTRAINT `aulas_ibfk_2` FOREIGN KEY (`curso_id`) REFERENCES `cursos` (`id`),
  ADD CONSTRAINT `aulas_ibfk_3` FOREIGN KEY (`componente_curricular_id`) REFERENCES `componentes_curriculares` (`id`);

ALTER TABLE `ciclos_letivos`
  ADD CONSTRAINT `ciclos_letivos_ibfk_1` FOREIGN KEY (`nivel_ensino_id`) REFERENCES `niveis_ensino` (`id`);

ALTER TABLE `componentes_curriculares`
  ADD CONSTRAINT `componentes_curriculares_ibfk_1` FOREIGN KEY (`curso_id`) REFERENCES `cursos` (`id`);

ALTER TABLE `cursos`
  ADD CONSTRAINT `cursos_ibfk_1` FOREIGN KEY (`nivel_ensino_id`) REFERENCES `niveis_ensino` (`id`);

ALTER TABLE `professors`
  ADD CONSTRAINT `user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);

ALTER TABLE `projetos_ensino`
  ADD CONSTRAINT `projetos_ensino_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);

ALTER TABLE `reunioes_formacao_docente`
  ADD CONSTRAINT `reunioes_formacao_docente_ibfk_1` FOREIGN KEY (`id_professor`) REFERENCES `professors` (`id`);
COMMIT;