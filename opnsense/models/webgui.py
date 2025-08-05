from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.protocol import Protocol
from opnsense.models.ssl_certref import SslCertref

__NAMESPACE__ = "https://opnsense.org/config"


class Webgui(BaseModel):
    class Meta:
        name = "webgui"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    protocol: Protocol = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ssl_certref: SslCertref | None = field(
        default=None,
        metadata={
            "name": "ssl-certref",
            "type": "Element",
            "namespace": "",
        },
    )
