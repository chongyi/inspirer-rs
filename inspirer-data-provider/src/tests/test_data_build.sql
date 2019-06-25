insert into users (user_uuid, mobile_phone, country_code, email, gender, user_type, nickname, password, last_login, last_login_ip, login_count, created_at, updated_at)
values ('b9e87a68d0dd4748806e7ddb403701f5', '18000000000', '86', 'administrator@test.com', true, 32767, 'administrator', '$2y$10$RfMto2wzx5bf7FtAFjXkHeRKDnJqBSEGnqIdknwcmQqQXrSd0XiLO', null, null, 0, '2019-06-01 00:00:00', '2019-06-01 00:00:00'),
       ('4bbd2cfbd5a04150985e34bb5b412b02', '18000000001', '86', 'nick@test.com', true, 1, 'nick', '$2y$10$RfMto2wzx5bf7FtAFjXkHeRKDnJqBSEGnqIdknwcmQqQXrSd0XiLO', null, null, 0, '2019-06-01 00:00:00', '2019-06-01 00:00:00'),
       ('c4881321dac844079842005bc9072256', '18000000002', '86', 'john@test.com', true, 1, 'john', '$2y$10$RfMto2wzx5bf7FtAFjXkHeRKDnJqBSEGnqIdknwcmQqQXrSd0XiLO', null, null, 0, '2019-06-01 00:00:00', '2019-06-01 00:00:00'),
       ('65086f5e6fee477c990c6bf4a09c7cb0', '18000000003', '86', 'alice@test.com', false, 1, 'alice', '$2y$10$RfMto2wzx5bf7FtAFjXkHeRKDnJqBSEGnqIdknwcmQqQXrSd0XiLO', null, null, 0, '2019-06-01 00:00:00', '2019-06-01 00:00:00'),
       ('c8200a9520a1470686b907ed89605735', '18000000004', '86', 'lisa@test.com', false, 1, 'lisa', '$2y$10$RfMto2wzx5bf7FtAFjXkHeRKDnJqBSEGnqIdknwcmQqQXrSd0XiLO', null, null, 0, '2019-06-01 00:00:00', '2019-06-01 00:00:00');

insert into validate_codes (code, validate_target, validate_channel, is_validated, expired_at)
                                                     values ('151010', '+86-18000000005', 1, false, '2019-06-01 00:00:00'),
                                                            ('151010', '+86-18000000006', 1, true, '2019-06-01 00:00:00'),
                                                            ('151010', 'foo@test.com', 2, false, '2019-06-01 00:00:00'),
                                                            ('151010', 'bar@test.com', 2, true, '2019-06-01 00:00:00'),
                                                            ('151010', '+86-18000000007', 1, false, null);