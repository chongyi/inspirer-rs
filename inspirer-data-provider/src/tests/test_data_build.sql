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

insert into contents (creator_uuid, title, content_type, published, display, published_at, created_at, updated_at)
values ('b9e87a68d0dd4748806e7ddb403701f5', 'This is a test title', 1, false, true, null, '2019-06-20 18:59:00', '2019-06-20 18:59:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Second test content', 1, true, true, '2019-06-20 19:00:00', '2019-06-20 19:00:00', '2019-06-20 19:00:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Suddenly she came upon a little three-legged table', 1, true, true, '2019-06-20 19:01:00', '2019-06-20 19:01:00', '2019-06-20 19:01:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Alice was not a bit hurt', 1, false, true, null, '2019-06-20 19:01:10', '2019-06-20 19:01:10'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'And so it was indeed', 1, true, true, '2019-06-20 19:02:00', '2019-06-20 19:02:00', '2019-06-20 19:02:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Curiouser and curiouser!', 1, true, true, '2019-06-20 19:03:00', '2019-06-20 19:03:00', '2019-06-20 19:03:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Oh! the Duchess, the Duchess!', 1, true, true, '2019-06-20 19:04:00', '2019-06-20 19:04:00','2019-06-20 19:04:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'After a while', 2, true, true, '2019-06-20 19:05:00', '2019-06-20 19:05:00', '2019-06-20 19:05:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Poor Alice!', 1, true, true, '2019-06-20 19:06:00', '2019-06-20 19:06:00', '2019-06-20 19:06:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'TOOK A WATCH OUT OF ITS WAISTCOAT-POCKET', 1, true, true, '2019-06-20 19:07:00', '2019-06-20 19:07:00', '2019-06-20 19:07:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Either the well was very deep', 1, true, true, '2019-06-20 19:08:00', '2019-06-20 19:08:00', '2019-06-20 19:08:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'ORANGE MARMALADE', 2, true, true, '2019-06-20 19:09:00', '2019-06-20 19:09:00', '2019-06-20 19:09:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'She ate a little bit', 1, true, true, '2019-06-20 19:10:00', '2019-06-20 19:10:00', '2019-06-20 19:10:00'),
       ('b9e87a68d0dd4748806e7ddb403701f5', 'Strange article', 1, true, true, '2019-06-20 19:11:00', '2019-06-20 19:11:00', '2019-06-20 19:11:00');
