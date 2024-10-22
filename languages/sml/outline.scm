(structure_strdec
    "structure" @context
    (strbind name: (strid) @name) @item)

(datatype_dec
    "datatype" @context
    (datbind name: (tycon) @name) @item)

(fun_dec
    "fun" @context
    (fvalbind (fmrule name: (vid) @name)) @item)

(signature_sigdec
    "signature" @context
    (sigbind name: (sigid) @name) @item)

(structure_spec
    "structure" @context
    (strdesc name: (strid) @name) @item)

(datatype_spec
    "datatype" @context
    (datdesc name: (tycon) @name) @item)

(val_spec
    "val" @context
    (valdesc name: (vid) @name) @item)
