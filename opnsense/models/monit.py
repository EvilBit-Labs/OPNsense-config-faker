from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.alert import Alert
from opnsense.models.general import General
from opnsense.models.service import Service
from opnsense.models.test import Test

__NAMESPACE__ = "https://opnsense.org/config"


class Monit(BaseModel):
    class Meta:
        name = "monit"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    alert: Alert = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    service: Iterable[Service] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
    test: Iterable[Test] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
