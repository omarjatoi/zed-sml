(_ "[" "]") @indent
(_ "{" "}") @indent
(_ "(" ")") @indent

[
    (fun_dec)
    (case_exp)
    (let_exp)
    (struct_strexp)
    (datatype_dec)
    (conbind)
    (app_exp)
    (mrule)
] @indent

(sig_sigexp
  "sig" @start
  "end" @end) @indent

(local_strdec
  "local" @start
  "in" @start
  "end" @end) @indent

"in" @indent
