from collections.abc import Iterable
from typing import Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.descr import Descr
from opnsense.models.key import Key
from opnsense.models.number import Number
from opnsense.models.secret import Secret
from opnsense.models.tunable import Tunable
from opnsense.models.type_mod import Type
from opnsense.models.value import Value

__NAMESPACE__ = "https://opnsense.org/config"


class Item(BaseModel):
    class Meta:
        name = "item"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    choice: Iterable[Descr | Tunable | Number | Type] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "descr",
                    "type": Descr,
                    "namespace": "",
                },
                {
                    "name": "tunable",
                    "type": Tunable,
                    "namespace": "",
                },
                {
                    "name": "number",
                    "type": Number,
                    "namespace": "",
                },
                {
                    "name": "type",
                    "type": Type,
                    "namespace": "",
                },
            ),
            "max_occurs": 2,
        },
    )
    value_or_key_or_secret: Iterable[Value | Key | Secret] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "value",
                    "type": Value,
                    "namespace": "",
                },
                {
                    "name": "key",
                    "type": Key,
                    "namespace": "",
                },
                {
                    "name": "secret",
                    "type": Secret,
                    "namespace": "",
                },
            ),
            "max_occurs": 2,
        },
    )
