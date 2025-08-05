from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.daemon import Daemon

__NAMESPACE__ = "https://opnsense.org/config"


class Syslog2(BaseModel):
    class Meta:
        name = "syslog"
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
                    "name": "daemon",
                    "type": Daemon,
                    "namespace": "",
                },
            ),
        },
    )
