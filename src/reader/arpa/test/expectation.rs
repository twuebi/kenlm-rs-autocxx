use crate::reader::arpa::{NGram, ProbBackoff, ProbBackoffNgram, ProbNgram};

macro_rules! prob_backoff_ngram {
    (
        $(
            $log_prob:expr, $ngram:expr, $backoff:expr
        );*
    ) => {

            vec![
                $(
                    ProbBackoffNgram {
                        prob_backoff: ProbBackoff {
                            log_prob: $log_prob,
                            backoff: $backoff,
                        },
                        ngram: NGram {
                            ngram: $ngram.to_string(),
                        },
                    }
                ),*
            ]
        }

}

macro_rules! prob_ngram {
    (
        $(
            $log_prob:expr, $ngram:expr
        );*
    ) => {

            vec![
                $(
                    ProbNgram {
                        prob: $log_prob,
                        ngram: NGram {
                            ngram: $ngram.to_string(),
                        },
                    }
                ),*
            ]
        }

}

#[allow(clippy::approx_constant)]
pub fn get_trigrams() -> Vec<ProbNgram> {
    prob_ngram!(-0.21873854 ,"a a </s>";
    -0.10757457,	"you remember i";
    -0.18978158, "<s> i have";
    -0.1770414,	"remember i a";
    -0.10225761,	"i have a";
    -0.2051335,	"i a a";
    -0.21873854,	"have a good";
    -0.112957425,	"a good deal";
    -0.112957425,	"good deal of";
    -0.112957425,	"deal of will";
    -0.112957425,	"of will you";
    -0.112957425,	"will you remember")
}

#[allow(clippy::approx_constant)]
pub fn get_bigrams() -> Vec<ProbBackoffNgram> {
    prob_backoff_ngram!(-0.68063426	,"a </s>",	-0.0;
    -0.250891	,"<s> i",	-0.30103;
    -0.250891	,"remember i",	-0.30103;
    -0.5346796	,"i have",	-0.30103;
    -0.4809342	,"i a",	-0.30103;
    -0.23625793	,"have a",	-0.30103;
    -0.6071514	,"a a",	-0.30103;
    -0.68063426	,"a good",	-0.30103;
    -0.26603433	,"good deal",	-0.30103;
    -0.26603433	,"deal of",	-0.30103;
    -0.26603433	,"of will",	-0.30103;
    -0.26603433	,"will you",	-0.30103;
    -0.26603433	,"you remember",	-0.30103)
}

#[allow(clippy::approx_constant)]
pub fn get_unigrams() -> Vec<ProbBackoffNgram> {
    prob_backoff_ngram!(-1.3424227,	"<unk>", -0.0;
    -0.0,           "<s>", -0.30103;
    -1.0761548,	"</s>", -0.0;
    -0.91229796,	"i", -0.30103;
    -1.0761548,	"have", -0.30103;
    -0.7936082,	"a", -0.30103;
    -1.0761548,	"good", -0.30103;
    -1.0761548,	"deal", -0.30103;
    -1.0761548,	"of", -0.30103;
    -1.0761548,	"will", -0.30103;
    -1.0761548,	"you", -0.30103;
    -1.0761548,	"remember", -0.30103)
}
