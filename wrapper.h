#include <ffnvcodec/nvEncodeAPI.h>

#define MARK_FIX_753(req_name) const unsigned long int Fix753_##req_name = req_name;

MARK_FIX_753(NV_ENC_CAPS_PARAM_VER);
