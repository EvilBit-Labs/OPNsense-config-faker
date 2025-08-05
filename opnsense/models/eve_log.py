from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.http import Http
from opnsense.models.tls import Tls

__NAMESPACE__ = "https://opnsense.org/config"


class EveLog(BaseModel):
    class Meta:
        name = "eveLog"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    http: Http = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    tls: Tls = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
