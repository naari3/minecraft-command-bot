use crate::minecraft_line::MinecraftLine;
use once_cell::sync::Lazy;
use regex::Regex;

use super::SendRule;

static DEATH_REGEXS: Lazy<Vec<Regex>> = Lazy::new(|| {
    let res = vec![
        r"(.*?)\swas\sshot\sby\s(.*?)",
        r"(.*?)\swas\sshot\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\spummeled\sby\s(.*?)",
        r"(.*?)\swas\spummeled\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\spricked\sto\sdeath",
        r"(.*?)\swalked\sinto\sa\scactus\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\sdrowned",
        r"(.*?)\sdrowned\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\sexperienced\skinetic\senergy",
        r"(.*?)\sexperienced\skinetic\senergy\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\sblew\sup",
        r"(.*?)\swas\sblown\sup\sby\s(.*?)",
        r"(.*?)\swas\sblown\sup\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\skilled\sby\s[Intentional\sGame\sDesign]",
        r"(.*?)\shit\sthe\sground\stoo\shard",
        r"(.*?)\shit\sthe\sground\stoo\shard\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\sfell\sfrom\sa\shigh\splace",
        r"(.*?)\sfell\soff\sa\sladder",
        r"(.*?)\sfell\soff\ssome\svines",
        r"(.*?)\sfell\soff\ssome\sweeping\svines",
        r"(.*?)\sfell\soff\ssome\stwisting\svines",
        r"(.*?)\sfell\soff\sscaffolding",
        r"(.*?)\sfell\swhile\sclimbing",
        r"(.*?)\swas\simpaled\son\sa\sstalagmite",
        r"(.*?)\swas\simpaled\son\sa\sstalagmite\swhilst\sfighting\s(.*?)",
        r"(.*?)\swas\ssquashed\sby\sa\sfalling\sanvil",
        r"(.*?)\swas\ssquashed\sby\sa\sfalling\sanvil\swhilst\sfighting\s(.*?)",
        r"(.*?)\swas\ssquashed\sby\sa\sfalling\sblock",
        r"(.*?)\swas\ssquashed\sby\sa\sfalling\sblock\swhilst\sfighting\s(.*?)",
        r"(.*?)\swas\sskewered\sby\sa\sfalling\sstalactite",
        r"(.*?)\swas\sskewered\sby\sa\sfalling\sstalactite\swhilst\sfighting\s(.*?)",
        r"(.*?)\swent\sup\sin\sflames",
        r"(.*?)\swalked\sinto\sfire\swhilst\sfighting\s(.*?)",
        r"(.*?)\sburned\sto\sdeath",
        r"(.*?)\swas\sburnt\sto\sa\scrisp\swhilst\sfighting\s(.*?)",
        r"(.*?)\swent\soff\swith\sa\sbang",
        r"(.*?)\swent\soff\swith\sa\sbang\sdue\sto\sa\sfirework\sfired\sfrom\s(.*?)\sby\s(.*?)",
        r"(.*?)\stried\sto\sswim\sin\slava",
        r"(.*?)\stried\sto\sswim\sin\slava\sto\sescape\s(.*?)",
        r"(.*?)\swas\sstruck\sby\slightning",
        r"(.*?)\swas\sstruck\sby\slightning\swhilst\sfighting\s(.*?)",
        r"(.*?)\sdiscovered\sthe\sfloor\swas\slava",
        r"(.*?)\swalked\sinto\sdanger\szone\sdue\sto\s(.*?)",
        r"(.*?)\swas\skilled\sby\smagic",
        r"(.*?)\swas\skilled\sby\smagic\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\swas\skilled\sby\s(.*?)\susing\smagic",
        r"(.*?)\swas\skilled\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\sfroze\sto\sdeath",
        r"(.*?)\swas\sfrozen\sto\sdeath\sby\s(.*?)",
        r"(.*?)\swas\sslain\sby\s(.*?)",
        r"(.*?)\swas\sslain\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\sfireballed\sby\s(.*?)",
        r"(.*?)\swas\sfireballed\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\sstung\sto\sdeath",
        r"(.*?)\swas\sshot\sby\sa\sskull\sfrom\s(.*?)",
        r"(.*?)\swas\sobliterated\sby\sa\ssonically-charged\sshriek",
        r"(.*?)\sstarved\sto\sdeath",
        r"(.*?)\sstarved\sto\sdeath\swhilst\sfighting\s(.*?)",
        r"(.*?)\ssuffocated\sin\sa\swall",
        r"(.*?)\ssuffocated\sin\sa\swall\swhilst\sfighting\s(.*?)",
        r"(.*?)\swas\ssquished\stoo\smuch",
        r"(.*?)\swas\ssquashed\sby\s(.*?)",
        r"(.*?)\swas\spoked\sto\sdeath\sby\sa\ssweet\sberry\sbush",
        r"(.*?)\swas\spoked\sto\sdeath\sby\sa\ssweet\sberry\sbush\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\swas\skilled\strying\sto\shurt\s(.*?)",
        r"(.*?)\swas\skilled\sby\s(.*?)\strying\sto\shurt\s(.*?)",
        r"(.*?)\swas\simpaled\sby\s(.*?)",
        r"(.*?)\swas\simpaled\sby\s(.*?)\swith\s(.*?)",
        r"(.*?)\sfell\sout\sof\sthe\sworld",
        r"(.*?)\sdidn't\swant\sto\slive\sin\sthe\ssame\sworld\sas\s(.*?)",
        r"(.*?)\swithered\saway",
        r"(.*?)\swithered\saway\swhilst\sfighting\s(.*?)",
        r"(.*?)\sdied\sfrom\sdehydration",
        r"(.*?)\sdied\sfrom\sdehydration\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\sdied",
        r"(.*?)\sdied\sbecause\sof\s(.*?)",
        r"(.*?)\swas\sroasted\sin\sdragon\sbreath",
        r"(.*?)\swas\sroasted\sin\sdragon\sbreath\sby\s(.*?)",
        r"(.*?)\swas\sdoomed\sto\sfall",
        r"(.*?)\swas\sdoomed\sto\sfall\sby\s(.*?)",
        r"(.*?)\swas\sdoomed\sto\sfall\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\sfell\stoo\sfar\sand\swas\sfinished\sby\s(.*?)",
        r"(.*?)\sfell\stoo\sfar\sand\swas\sfinished\sby\s(.*?)\susing\s(.*?)",
        r"(.*?)\swas\sstung\sto\sdeath\sby\s(.*?)",
        r"(.*?)\swent\soff\swith\sa\sbang\swhilst\sfighting\s(.*?)",
        r"(.*?)\swas\sobliterated\sby\sa\ssonically-charged\sshriek\swhilst\strying\sto\sescape\s(.*?)",
        r"(.*?)\swas\skilled\sby\seven\smore\smagic",
    ];
    res.into_iter().map(|re| Regex::new(re).unwrap()).collect()
});

#[derive(Clone)]
pub struct DeathRule;

impl SendRule for DeathRule {
    fn send(&self, line: &MinecraftLine) -> Option<String> {
        if !line.caused_at.contains("Server thread") || !line.level.eq("INFO") {
            return None;
        }

        for re in DEATH_REGEXS.iter() {
            if re.is_match(&line.message) {
                return Some(format!("**{}**", line.message.clone()));
            }
        }
        None
    }
}
