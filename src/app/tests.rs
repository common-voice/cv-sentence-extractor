use super::*;
use std::path::PathBuf;

#[test]
fn test_extractor() {
    let texts = load(&PathBuf::from("src/test_data/wiki_00")).unwrap();
    let mut builder = SentenceExtractorBuilder::new();
    let mut iter = builder.build(texts[0].as_str());

    assert_eq!(iter.next().unwrap(), "愛因斯坦");
    assert_eq!(iter.next().unwrap(), "全名音譯");
    assert_eq!(iter.next().unwrap(), "阿爾拔·愛因斯坦");
    assert_eq!(iter.next().unwrap(), "係一位理論物理學家");
    assert_eq!(iter.next().unwrap(), "佢最出名嘅係發表咗相對論");
    assert_eq!(iter.next().unwrap(), "另外喺量子力學");
    assert_eq!(iter.next().unwrap(), "統計力學");
    assert_eq!(iter.next().unwrap(), "同埋宇宙學方面都有好大貢獻");
    assert_eq!(iter.next().unwrap(), "愛因斯坦喺德國烏爾姆市出世");
    assert_eq!(iter.next().unwrap(), "一年後成家人搬咗去慕尼黑");
    assert_eq!(iter.next().unwrap(), "佢屋企都係猶太人");
    assert_eq!(iter.next().unwrap(), "但係冇入猶太教");
    assert_eq!(iter.next().unwrap(), "佢爸爸本來賣床褥");
    assert_eq!(iter.next().unwrap(), "後來開電器舖");
    assert_eq!(iter.next().unwrap(), "五歲嗰年");
    assert_eq!(iter.next().unwrap(), "佢爸爸送咗個指南針畀佢");
    assert_eq!(iter.next().unwrap(), "佢就發現有啲睇唔到嘅嘢牽引住枝針");
    assert_eq!(
        iter.next().unwrap(),
        "後來愛因斯坦話嗰次嘅經驗係佢一生中最有啟發性"
    );
    assert_eq!(iter.next().unwrap(), "雖然佢識砌啲機械模型嚟玩");
    assert_eq!(iter.next().unwrap(), "但係讀書讀得好慢");
    assert_eq!(iter.next().unwrap(), "可能係因為學習障礙病");
    assert_eq!(iter.next().unwrap(), "又或者只係因為怕醜");
    assert_eq!(iter.next().unwrap(), "又或者係因為佢個腦結構特殊");
    assert_eq!(
        iter.next().unwrap(),
        "最新嘅理論話愛因斯坦應該係患咗亞氏保加症"
    );
    assert_eq!(iter.next().unwrap(), "係自閉症嘅一種");
    assert_eq!(iter.next().unwrap(), "因為當時呢個病未畀人發現");
    assert_eq!(iter.next().unwrap(), "佢父母重以為佢係低能");
    assert_eq!(iter.next().unwrap(), "愛因斯坦話自己之所以諗得出相對論");
    assert_eq!(iter.next().unwrap(), "正係因為細個時學嘢慢");
    assert_eq!(iter.next().unwrap(), "遲過其他小朋友開始思索時空");
    assert_eq!(iter.next().unwrap(), "到嗰陣思想已經比較成熟");
    assert_eq!(iter.next().unwrap(), "因為佢成功發現光電效應");
    assert_eq!(iter.next().unwrap(), "後來佢又寫咗好多有關時空");
    assert_eq!(iter.next().unwrap(), "物質嘅理論");
    assert_eq!(iter.next().unwrap(), "因為當時嘅人睇唔明佢嘅理論");
    assert_eq!(iter.next().unwrap(), "導致佢無法得到應有嘅尊重");
    assert_eq!(iter.next().unwrap(), "佢嘅理論");
    assert_eq!(iter.next().unwrap(), "先有人開始明白少少");
    assert_eq!(iter.next().unwrap(), "但至今");
    assert_eq!(iter.next().unwrap(), "重有好多都睇唔明佢寫乜");
    assert_eq!(iter.next().unwrap(), "最大唔同嘅係");
    assert_eq!(iter.next().unwrap(), "人已經尊重佢");
    assert_eq!(iter.next().unwrap(), "而唔係當佢癲佬");
    assert!(iter.next().is_none());
}

#[test]
fn test_extractor_with_bondary_condition() {
    let texts = load(&PathBuf::from("src/test_data/wiki_01")).unwrap();
    let mut builder = SentenceExtractorBuilder::new()
        .shortest_length(1)
        .longest_length(1);

    let mut iter = builder.build(texts[0].as_str());
    assert_eq!(iter.next().unwrap(), "春");
    assert_eq!(iter.next().unwrap(), "多");
    assert_eq!(iter.next().unwrap(), "國");
    assert_eq!(iter.next().unwrap(), "玉？");
    assert_eq!(iter.next().unwrap(), "砌！");
    assert_eq!(iter.next().unwrap(), "應");
    assert_eq!(iter.next().unwrap(), "猶");
    assert!(iter.next().is_none());
}

#[test]
fn test_extractor_with_ignore_symbols() {
    let texts = load(&PathBuf::from("src/test_data/wiki_02")).unwrap();
    let ignore_symbols = vec!['「', '」'];
    let mut builder = SentenceExtractorBuilder::new().ignore_symbols(&ignore_symbols);
    let mut iter = builder.build(texts[0].as_str());

    assert_eq!(iter.next().unwrap(), "噬魂師");
    assert_eq!(iter.next().unwrap(), "係由大久保篤創作嘅日本漫畫作品");
    assert_eq!(iter.next().unwrap(), "舞台為死神武器工匠專門學校");
    assert_eq!(iter.next().unwrap(), "俗稱死武專");
    assert_eq!(iter.next().unwrap(), "呢間學校係專門培育工匠同武器");
    assert_eq!(iter.next().unwrap(), "工匠同武器係一對嘅");
    assert_eq!(
        iter.next().unwrap(),
        "工匠同武器嘅靈魂波長配合會令到戰鬥力提高"
    );
    assert_eq!(iter.next().unwrap(), "工匠參與實戰");
    assert_eq!(
        iter.next().unwrap(),
        "武器則會變化成自己擅長形態嘅武器支援工匠"
    );
    assert_eq!(iter.next().unwrap(), "武器可以控制同支援工匠靈魂波長嘅增強");
    assert_eq!(iter.next().unwrap(), "工匠就擁有探測靈魂種類同位置");
    assert!(iter.next().is_none());
}

#[test]
fn test_extractor_with_ending_symbols() {
    let texts = load(&PathBuf::from("src/test_data/wiki_01")).unwrap();
    let mut builder = SentenceExtractorBuilder::new();
    let mut iter = builder.build(texts[0].as_str());
    assert_eq!(iter.next().unwrap(), "月何時");
    assert_eq!(iter.next().unwrap(), "了往事知");
    assert_eq!(iter.next().unwrap(), "樓昨夜");
    assert_eq!(iter.next().unwrap(), "又東風故");
    assert_eq!(iter.next().unwrap(), "回首月");
    assert_eq!(iter.next().unwrap(), "明中雕欄");
    assert_eq!(iter.next().unwrap(), "在「只」是《朱》顏");
    assert_eq!(iter.next().unwrap(), "改『問』君【能】有…幾—多．愁");
    assert!(iter.next().is_none());
}

#[test]
fn test_extractor_with_black_list_symbols() {
    let texts = load(&PathBuf::from("src/test_data/wiki_01")).unwrap();
    let black_list_symbols = vec!['《', '》'];
    let mut builder = SentenceExtractorBuilder::new().black_list_symbols(&black_list_symbols);
    let mut iter = builder.build(texts[0].as_str());
    assert_eq!(iter.next().unwrap(), "月何時");
    assert_eq!(iter.next().unwrap(), "了往事知");
    assert_eq!(iter.next().unwrap(), "樓昨夜");
    assert_eq!(iter.next().unwrap(), "又東風故");
    assert_eq!(iter.next().unwrap(), "回首月");
    assert_eq!(iter.next().unwrap(), "明中雕欄");
    assert_eq!(iter.next().unwrap(), "改『問』君【能】有…幾—多．愁");
    assert!(iter.next().is_none());
}
