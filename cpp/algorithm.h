class Algorithm
{
 public:
  Algorithm() = default;

  virtual void OnRegister() = 0;
  virtual void OnSystemStart() = 0;
  virtual void MainThreadEvent() = 0;
  virtual void BackgroundThreadEvent() = 0;
  virtual void OnSystemStop() = 0;
  virtual void OnUnregister() = 0;
};
