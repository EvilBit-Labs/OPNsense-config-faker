from collections.abc import Iterable
from typing import Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.code import Code
from opnsense.models.expect import Expect
from opnsense.models.host import Host
from opnsense.models.path import Path
from opnsense.models.send import Send

__NAMESPACE__ = "https://opnsense.org/config"


class Options(BaseModel):
    class Meta:
        name = "options"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    choice: Iterable[Path | Host | Code | Send | Expect] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "path",
                    "type": Path,
                    "namespace": "",
                },
                {
                    "name": "host",
                    "type": Host,
                    "namespace": "",
                },
                {
                    "name": "code",
                    "type": Code,
                    "namespace": "",
                },
                {
                    "name": "send",
                    "type": Send,
                    "namespace": "",
                },
                {
                    "name": "expect",
                    "type": Expect,
                    "namespace": "",
                },
            ),
            "max_occurs": 3,
        },
    )
