from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Numqueriesperthread(BaseModel):
    class Meta:
        name = "numqueriesperthread"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
