from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Logservfail(BaseModel):
    class Meta:
        name = "logservfail"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
