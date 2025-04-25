# local issues lib

## changes between v0.1.1 ~ v0.2.0

- Issueの`body`を`Vec<String>`に


## RoadMap

### v0.1

5月始まりまでにv0.1リリース予定。

- [ ] Issue管理
  - [x] idの取得
    - [x] titleとの完全一致
    - [x] 部分一致
  - [x] 追加
  - [x] 削除
    - [x] idで削除
    - [x] titleで完全一致が一つだった場合に削除
    - [x] statusがMarkedAsDeletedかつ0のときに自動削除(save()に入ってる)
  - [ ] 編集
    - [x] タイトル
      - [x] idから
      - [ ] ~titleから~
        - [x] 一意に定まらないからcli(or gui)側で実装
    - [ ] タグ
    - [ ] `due`日時
    - [ ] status
    - [ ] `body`パス
  - [ ] `body`パスからファイルの作成
  - [ ] `updated_at`などの自動更新
  - [ ] 各フィールドを条件に検索(Vec<>で返す)
- [ ] Project管理
  - [x] Projectのopen
  - [x] 👆のsave
  - [ ] 編集
    - [ ] 名前
    - [ ] タグ
    - [ ] `db_path`
    - [ ] `work_path`

### v0.2(予定)

5月終わりくらい?GW?

- [ ] `body`へfrontmatter
- [ ] `body`の`h1`tag(`# title`)と`Issue::title`の同期
- [ ] 公式(?)でcli, gui版の提供ができれば万々歳
- [ ] body path(PathbBuf)の変更でも`update_at`を変更
- [ ] deleteフラグなどの値をconfigで管理可能に

- get_tags
edit_status
update_date