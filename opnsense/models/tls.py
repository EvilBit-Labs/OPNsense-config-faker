from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.custom import Custom
from opnsense.models.enable import Enable
from opnsense.models.extended import Extended
from opnsense.models.session_resumption import SessionResumption

__NAMESPACE__ = "https://opnsense.org/config"


class Tls(BaseModel):
    class Meta:
        name = "tls"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    content: Iterable[object] = field(
        default_factory=list,
        metadata={
            "type": "Wildcard",
            "namespace": "##any",
            "mixed": True,
            "choices": (
                {
                    "name": "enable",
                    "type": Enable,
                    "namespace": "",
                },
                {
                    "name": "extended",
                    "type": Extended,
                    "namespace": "",
                },
                {
                    "name": "custom",
                    "type": Custom,
                    "namespace": "",
                },
                {
                    "name": "sessionResumption",
                    "type": SessionResumption,
                    "namespace": "",
                },
            ),
        },
    )
