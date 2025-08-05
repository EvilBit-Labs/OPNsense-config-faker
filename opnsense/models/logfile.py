from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Logfile(BaseModel):
    class Meta:
        name = "logfile"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
