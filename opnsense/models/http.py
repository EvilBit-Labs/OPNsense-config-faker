from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.dump_all_headers import DumpAllHeaders
from opnsense.models.enable import Enable
from opnsense.models.extended import Extended

__NAMESPACE__ = "https://opnsense.org/config"


class Http(BaseModel):
    class Meta:
        name = "http"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enable: Enable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    extended: Extended = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dump_all_headers: DumpAllHeaders = field(
        metadata={
            "name": "dumpAllHeaders",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
