from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class DumpAllHeaders(BaseModel):
    class Meta:
        name = "dumpAllHeaders"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
