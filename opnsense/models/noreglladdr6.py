from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Noreglladdr6(BaseModel):
    class Meta:
        name = "noreglladdr6"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
